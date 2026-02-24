import { useState } from "react";
import { getProfileImgUrl } from "../lib/helpers";
import Camera from "../SVGS/Camera";
import Image from "next/image";
import UpdateImageForm from "./UpdateImageForm";
interface AvatarProps {
  image: string | null;
}
export default function Avatar({ image }: AvatarProps) {
  const [showImageForm, setShowImageForm] = useState(false);
  const imageUrl = image ? getProfileImgUrl(image) : null;
  return (
    <>
      <div className="relative shrink-0">
        <div className="flex h-[110px] w-[110px] items-center justify-center overflow-hidden rounded-full border-3 border-secondary/20 bg-[linear-gradient(135deg,var(--color-secondary)/0.25,var(--color-primary)/0.7)] shadow-[0_8px_32px_rgba(0,0,0,0.3)]">
          {imageUrl && (
            <Image
              width={200}
              height={200}
              src={imageUrl ?? "/"}
              className="h-full w-full object-cover"
              alt="Profile"
              unoptimized
            />
          )}
        </div>

        <button
          type="button"
          aria-label="Update profile picture"
          onClick={() => setShowImageForm(true)}
          title="Update profile picture"
          className="absolute -bottom-1 -right-1 flex h-[34px] w-[34px] cursor-pointer items-center justify-center rounded-full border-3 border-primary bg-[linear-gradient(135deg,var(--color-secondary),var(--color-support))] text-primary transition-transform duration-200 ease-in-out hover:scale-110"
        >
          <Camera />
        </button>
      </div>
      {showImageForm && (
        <UpdateImageForm
          currentImageName={image}
          onClose={() => setShowImageForm(false)}
        />
      )}
    </>
  );
}
